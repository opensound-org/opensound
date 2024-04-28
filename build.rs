fn main() -> Result<(), ()> {
    let is_zh_cn = sys_locale::get_locale() == Some("zh-CN".into());
    let mut num = 0;

    for flag in [
        "ACTIX_WEB",
        "AXUM",
        "NTEX",
        "POEM",
        "ROCKET",
        "SALVO",
        "VIZ",
        "WARP",
    ] {
        if std::env::var(format!("CARGO_FEATURE_{}", flag)).is_ok() {
            num += 1;
        }
    }

    if num != 1 {
        let features =
            "\"actix-web\", \"axum\", \"ntex\", \"poem\", \"rocket\", \"salvo\", \"viz\", \"warp\"";

        if is_zh_cn {
            eprintln!("以下feature中有且仅有一个可以被启用：{}。", features);
        } else {
            eprintln!(
                "One and only one of the following features can be enabled: {}.",
                features
            );
        }

        if num == 0 {
            if is_zh_cn {
                eprintln!("您一个也没有启用它们。");
            } else {
                eprintln!("You haven't enabled any of them.");
            }
        } else {
            if is_zh_cn {
                eprintln!("您启用了它们中的多个。");
            } else {
                eprintln!("You have multiple of them enabled.");
            }
        }

        return Err(());
    }

    Ok(())
}
