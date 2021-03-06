extern crate gcc;

fn main() {
    gcc::Config::new()
        .cpp(true)
        .file("../third-party/cimgui/cimgui/cimgui.cpp")
        .file("../third-party/cimgui/cimgui/fontAtlas.cpp")
        .file("../third-party/cimgui/cimgui/drawList.cpp")
        .file("../third-party/cimgui/imgui/imgui.cpp")
        .file("../third-party/cimgui/imgui/imgui_demo.cpp")
        .file("../third-party/cimgui/imgui/imgui_draw.cpp")
        .compile("libcimgui.a");
}
