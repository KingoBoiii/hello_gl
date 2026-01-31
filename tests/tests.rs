#[cfg(test)]
mod tests {
    extern crate glfw;
    extern crate gl;
    
    #[test]
    fn test_glfw_initialization() {
        let glfw = glfw::init(glfw::FAIL_ON_ERRORS);
        assert!(glfw.is_ok(), "GLFW initialization failed");
    }
}

