use futures::StreamExt;
use ytextract::Client;

#[tokio::test]
async fn get() -> Result<(), Box<dyn std::error::Error>> {
    let video = Client::new()
        .video("https://www.youtube.com/watch?v=7B2PIVSWtJA".parse()?)
        .await?;

    assert_eq!(
        video.title(),
        "I Sent Corridor Digital the WORST VFX Workstation"
    );

    assert_eq!(video.id(), "7B2PIVSWtJA".parse()?);
    assert_eq!(video.duration(), std::time::Duration::from_secs(1358));
    assert_eq!(
        video.keywords(),
        &vec![
            "photoshop",
            "adobe",
            "1.0",
            "macintosh",
            "apple",
            "lc",
            "475",
            "quadra",
            "performa",
            "classic",
            "system 7.5",
            "macos",
            "ossc",
            "vga",
            "vfx",
            "editing",
            "challenge",
            "corridor digital",
            "collab",
            "ftp",
            "fetch",
            "icab",
            "marathon",
            "oregon trail",
            "nightmare fuel",
            "scsi2sd"
        ]
    );
    let channel = video.channel();
    assert_eq!(channel.id(), "UCXuqSBlHAE6Xw-yeJA0Tunw".parse()?);
    assert_eq!(channel.name(), "Linus Tech Tips");
    assert!(channel.subscribers() >= Some(13_800_000));
    assert!(channel.thumbnails().next().is_some());
    assert_eq!(channel.id(), channel.upgrade().await?.id());
    assert!(!video.description().is_empty());
    assert!(video.views() >= 1_068_917);
    //assert!(video.likes() >= Some(51_745));
    assert!(!video.live());
    assert!(!video.thumbnails().is_empty());
    assert_eq!(video.date(), chrono::NaiveDate::from_ymd(2021, 4, 14));
    assert!(video.hashtags().next().is_none());

    let mut streams = video.streams().await?;
    assert!(streams.next().is_some());

    Ok(())
}

#[tokio::test]
async fn eq() -> Result<(), Box<dyn std::error::Error>> {
    let video1 = Client::new().video("7B2PIVSWtJA".parse()?).await?;
    let video2 = Client::new().video("7B2PIVSWtJA".parse()?).await?;

    assert_eq!(video1, video2);

    Ok(())
}

#[tokio::test]
async fn eq_channel() -> Result<(), Box<dyn std::error::Error>> {
    let video1 = Client::new().video("7B2PIVSWtJA".parse()?).await?;
    let video2 = Client::new().video("7B2PIVSWtJA".parse()?).await?;

    assert_eq!(video1.channel(), video2.channel());

    Ok(())
}

#[tokio::test]
async fn likes_not_allowed() -> Result<(), Box<dyn std::error::Error>> {
    let video = Client::new().video("9Jg_Fwc0QOY".parse()?).await?;
    assert_eq!(video.likes(), None);

    Ok(())
}

mod metadata {
    use ytextract::Client;

    macro_rules! define_test {
        ($fn:ident, $id:literal) => {
            #[tokio::test]
            async fn $fn() -> Result<(), Box<dyn std::error::Error>> {
                let id = $id.parse()?;
                let video = Client::new().video(id).await?;
                assert_eq!(video.id(), id);
                Ok(())
            }
        };
    }

    define_test!(normal, "9bZkp7q19f0");
    define_test!(live_stream, "5qap5aO4i9A");
    define_test!(live_stream_recording, "rsAAeyAr-9Y");
    define_test!(high_quality_streams, "V5Fsj_sCKdg");
    define_test!(vr, "-xNN-bJQ4vI");
    define_test!(hdr, "vX2vsvdq8nw");
    define_test!(age_restricted, "SkRSXFQerZs");
    define_test!(required_purchase, "p3dDcKOFXQg");
    define_test!(subtitles, "YltHGKX80Y8");
    define_test!(premire, "vv-Fqm6Qtj4");

    mod embed_restricted {
        use ytextract::Client;

        define_test!(youtube, "_kmeFXjjGfk");
        define_test!(author, "MeJVWBSsPAY");
        define_test!(age_restricted, "hySoCSoH-g8");
    }
}

#[tokio::test]
async fn related() -> Result<(), Box<dyn std::error::Error>> {
    let id = "9bZkp7q19f0".parse()?;

    let video = Client::new().video(id).await?;

    video
        .related()
        .expect("No related videos found")
        .take(100)
        .collect::<Vec<_>>()
        .await;

    Ok(())
}
