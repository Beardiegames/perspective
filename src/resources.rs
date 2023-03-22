use crate::*;

pub async fn load_binary(file_name: &str) -> anyhow::Result<Vec<u8>> {

    let path = std::path::Path::new("assets")
        //.join("res")
        .join(file_name);
    let data = std::fs::read(path)?;
    Ok(data)
}

pub async fn load_texture(file_name: &str, canvas: &Canvas) -> anyhow::Result<Texture> {
    let data = load_binary(file_name).await?;
    Texture::from_bytes(canvas, &data, file_name)
}
