mod graphics;

#[cfg(test)]
pub mod tests {
    use geop_wgpu::headless_renderer::HeadlessRenderer;
    use rstest::fixture;

    #[fixture]
    pub async fn renderer() -> Box<HeadlessRenderer> {
        Box::new(HeadlessRenderer::new().await)
    }
}
