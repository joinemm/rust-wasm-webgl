pub const SHADER: &str = r#"

    attribute vec4 position;
    uniform mat4 uTransform;

    void main() {
        gl_position = uTransform * aPosition;
    }

"#;
