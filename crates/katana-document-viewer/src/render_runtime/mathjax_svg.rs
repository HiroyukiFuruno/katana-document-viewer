use rquickjs::{CatchResultExt, Context, Ctx, Function, Object, Runtime};
use std::cell::RefCell;
use std::panic::{AssertUnwindSafe, catch_unwind};

const EX_TO_PX: f32 = 8.5;
const EXPORT_SUFFIX: &str = "export{Nj as default};";
const FUNC_ID: &str = "__kdv_mathjax_render";
const QUICKJS_STACK_LIMIT_BYTES: usize = 8 * 1024 * 1024;

thread_local! {
    static MATHJAX_CONTEXT: RefCell<Option<MathJaxContext>> = const { RefCell::new(None) };
}

struct MathJaxContext {
    _runtime: Runtime,
    context: Context,
}

pub(crate) struct MathJaxSvgRenderer;

impl MathJaxSvgRenderer {
    pub(crate) fn render(expression: &str, inline: bool) -> Result<String, String> {
        catch_unwind(AssertUnwindSafe(|| Self::render_inner(expression, inline)))
            .map_err(|_| "math SVG renderer panicked".to_string())?
    }

    fn render_inner(expression: &str, inline: bool) -> Result<String, String> {
        render_mathjax_svg(expression.trim(), !inline).map(normalize_svg_size)
    }
}

fn render_mathjax_svg(expression: &str, display: bool) -> Result<String, String> {
    MATHJAX_CONTEXT.with(|context_cell| {
        let mut context_slot = context_cell.borrow_mut();
        if context_slot.is_none() {
            *context_slot = Some(initialize_mathjax()?);
        }
        let context = context_slot
            .as_ref()
            .ok_or_else(|| "MathJax JavaScript context was not initialized".to_string())?;
        context.context.with(|ctx| {
            let config = catch_js(&ctx, Object::new(ctx.clone()))?;
            catch_js(&ctx, config.set("display", display))?;
            let render: Function = catch_js(&ctx, ctx.globals().get(FUNC_ID))?;
            catch_js(&ctx, render.call((expression, config)))
        })
    })
}

fn initialize_mathjax() -> Result<MathJaxContext, String> {
    let runtime = Runtime::new().map_err(|error| error.to_string())?;
    runtime.set_max_stack_size(QUICKJS_STACK_LIMIT_BYTES);
    let context = Context::full(&runtime).map_err(|error| error.to_string())?;
    context.with(|ctx| catch_js(&ctx, ctx.eval::<(), _>(patched_bundle()?.as_str())))?;
    Ok(MathJaxContext {
        _runtime: runtime,
        context,
    })
}

fn catch_js<'js, T>(ctx: &Ctx<'js>, result: rquickjs::Result<T>) -> Result<T, String> {
    result.catch(ctx).map_err(|error| error.to_string())
}

fn patched_bundle() -> Result<String, String> {
    let source = include_str!("../mathjax_bundle/index.mjs");
    let export_start = source
        .rfind(EXPORT_SUFFIX)
        .ok_or_else(|| "MathJax bundle export marker was not found".to_string())?;
    let export_end = export_start + EXPORT_SUFFIX.len();
    if !source[export_end..].trim().is_empty() {
        return Err("MathJax bundle has unexpected content after export marker".to_string());
    }
    let script = &source[..export_start];
    Ok(format!("{script}globalThis.{FUNC_ID}=Nj;"))
}

fn normalize_svg_size(svg: String) -> String {
    let svg = normalize_dimension_attribute(svg, "width");
    normalize_dimension_attribute(svg, "height")
}

fn normalize_dimension_attribute(mut svg: String, name: &str) -> String {
    let mut search_start = 0usize;
    while let Some(attribute_start) = svg[search_start..].find(&format!("{name}=\"")) {
        let value_start = search_start + attribute_start + name.len() + 2;
        let Some(value_end_offset) = svg[value_start..].find('"') else {
            break;
        };
        let value_end = value_start + value_end_offset;
        let value = &svg[value_start..value_end];
        let Some(ex_value) = value.strip_suffix("ex") else {
            search_start = value_end + 1;
            continue;
        };
        let Ok(number) = ex_value.parse::<f32>() else {
            search_start = value_end + 1;
            continue;
        };
        let replacement = format!("{:.2}px", number * EX_TO_PX);
        svg.replace_range(value_start..value_end, &replacement);
        search_start = value_start + replacement.len();
    }
    svg
}
