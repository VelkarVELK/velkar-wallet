use velkar_cli_lib::velkar_cli;
use wasm_bindgen::prelude::*;
use workflow_terminal::Options;
use workflow_terminal::Result;

#[wasm_bindgen]
pub async fn load_velkar_wallet_cli() -> Result<()> {
    let options = Options { ..Options::default() };
    velkar_cli(options, None).await?;
    Ok(())
}
