pub fn is_same_size(a: &impl image::GenericImage, b: &impl image::GenericImage) -> bool {
    a.width() == b.width() && a.height() == b.height()
}
