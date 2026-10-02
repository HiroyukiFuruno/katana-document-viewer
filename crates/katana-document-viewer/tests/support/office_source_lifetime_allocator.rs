use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

pub const RELEASE_MARKER: &str = "[KDV_LIFETIME_TEST] original_source_dealloc";
static WATCHED: AtomicUsize = AtomicUsize::new(0);
static WRITE_FAILED: AtomicBool = AtomicBool::new(false);

pub fn watch(bytes: &[u8]) {
    WATCHED.store(bytes.as_ptr() as usize, Ordering::SeqCst);
}

pub fn marker_write_failed() -> bool {
    WRITE_FAILED.load(Ordering::SeqCst)
}

#[cfg(unix)]
unsafe extern "C" {
    fn write(fd: i32, bytes: *const u8, count: usize) -> isize;
}

#[cfg(windows)]
#[link(name = "ucrt")]
unsafe extern "C" {
    fn _write(fd: i32, bytes: *const u8, count: u32) -> i32;
}

fn record_release(pointer: *mut u8) {
    if WATCHED
        .compare_exchange(pointer as usize, 0, Ordering::SeqCst, Ordering::SeqCst)
        .is_err()
    {
        return;
    }
    const ROW: &[u8] = b"[KDV_LIFETIME_TEST] original_source_dealloc\n";
    // allocator内の再入を避け、固定byte列だけを有効なstderrへ書き込む。
    #[cfg(unix)]
    let written = unsafe { write(2, ROW.as_ptr(), ROW.len()) };
    // Windows CRTのcountは32bitで、この固定byte列の長さはその範囲内。
    #[cfg(windows)]
    let written = unsafe { _write(2, ROW.as_ptr(), ROW.len() as u32) };
    WRITE_FAILED.store(written as usize != ROW.len(), Ordering::SeqCst);
}

struct ObservedSystem;

// テストのptr/Layoutを変更せずSystemへ委譲し、原本解放だけを観測する。
unsafe impl GlobalAlloc for ObservedSystem {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // 元の有効なLayoutと所有権をそのまま維持する。
        unsafe { System.alloc(layout) }
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        // zeroed契約を変更せずSystemへ委譲する。
        unsafe { System.alloc_zeroed(layout) }
    }

    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        record_release(pointer);
        // Systemが返したptrを元のLayoutで解放する。
        unsafe { System.dealloc(pointer, layout) };
    }

    unsafe fn realloc(&self, pointer: *mut u8, layout: Layout, size: usize) -> *mut u8 {
        // 再割当の失敗時も元割当が有効というSystemの契約を保持する。
        unsafe { System.realloc(pointer, layout, size) }
    }
}

#[global_allocator]
static ALLOCATOR: ObservedSystem = ObservedSystem;
