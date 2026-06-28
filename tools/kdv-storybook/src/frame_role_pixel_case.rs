pub(crate) struct FixtureRolePixelCase {
    pub(crate) fixture: &'static str,
    pub(crate) roles: Vec<RolePixelExpectation>,
}

pub(crate) struct RolePixelExpectation {
    pub(crate) name: &'static str,
    pub(crate) pixel_color: u32,
    pub(crate) minimum_nodes: usize,
    pub(crate) minimum_pixels: usize,
}

impl RolePixelExpectation {
    pub(crate) fn new(
        name: &'static str,
        pixel_color: u32,
        minimum_nodes: usize,
        minimum_pixels: usize,
    ) -> Self {
        Self {
            name,
            pixel_color,
            minimum_nodes,
            minimum_pixels,
        }
    }
}
