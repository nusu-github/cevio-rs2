pub use cevio::*;

mod cevio;

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use anyhow::Result;

    use super::*;

    #[test]
    // 公式のサンプル
    // https://cevio.jp/guide/cevio_ai/interface/com/
    fn minimal() -> Result<()> {
        let cevio = Cevio::new()?;

        // 【CeVIO AI】起動
        cevio.start(false)?;

        let cast = CastBuilder::default()
            // キャスト設定
            .cast("さとうささら")
            // 音量設定
            .volume(100)
            // 抑揚設定
            .tone_scale(100)
            .build()?;

        cevio.apply_cast(&cast)?;

        // （例）再生
        let e = cevio.speak("こんにちは")?;
        e.wait()?;

        // （例）音素データ取得
        let phonemes = cevio.phonemes("はじめまして")?;
        println!("{:?}", phonemes);

        // 【CeVIO AI】終了
        cevio.close(CloseMode::Interactive)?;

        Ok(())
    }

    #[test]
    fn full() -> Result<()> {
        let cevio = Cevio::new()?;

        // 【CeVIO AI】起動
        cevio.start(false)?;

        // キャスト一覧
        let components = cevio.available_casts()?;

        for component in components {
            let start = Instant::now();
            println!("{:?}", component);

            let cast = CastBuilder::default()
                // キャスト設定
                .cast(component)
                // （例）音量設定
                .volume(100)
                // （例）抑揚設定
                .tone_scale(100)
                .build()?;

            cevio.apply_cast(&cast)?;

            // 感情パラメータ取得
            let components = cevio.components()?;
            println!("{:?}", components);

            // （例）再生
            let e = cevio.speak("こんにちは")?;
            e.wait()?;

            // （例）音素データ取得
            let phonemes = cevio.phonemes("はじめまして")?;
            println!("{:?}", phonemes);

            println!("{}ms", start.elapsed().as_millis());
        }

        // 【CeVIO AI】終了
        cevio.close(CloseMode::Interactive)?;

        Ok(())
    }
}
