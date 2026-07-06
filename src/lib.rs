pub mod status;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn status() {
        assert_eq!(status::status(), "MOSAIC-UMD called");
    }
}
