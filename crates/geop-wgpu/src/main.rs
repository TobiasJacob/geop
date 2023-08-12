use geop_wgpu::window::GeopWindow;


async fn run() {
    let window = GeopWindow::new().await;
    window.show();
}

fn main() {
    pollster::block_on(run());
}