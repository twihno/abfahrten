use anyhow::anyhow;

use reqwest::header::{HeaderMap, HeaderValue};

pub struct SpielplanElement {}

pub async fn fetch_spielplan_spiegel(year: u16) -> anyhow::Result<String> {
    let res = reqwest::Client::new()
        .get(&format!(
            "https://sportdaten.spiegel.de/spielort/ve81/allianz-arena/vs{}/spielplan/",
            year
        ))
        .send()
        .await?;

    let html_content = res.text().await?;

    let wrapper_index = html_content
        .find("class=\"module-gameplan\"")
        .ok_or(anyhow!("AAA"))?;
    let mut remaining_content = html_content.split_at(wrapper_index).1;

    loop {
        remaining_content = remaining_content
            .split_at(html_content.find("class=\"hs-head").ok_or(anyhow!("Bbb"))?)
            .1;

        let name_start = remaining_content.find(">").ok_or(anyhow!("aaaad"))? + 1;
        let name_len = remaining_content
            .split_at(name_start)
            .1
            .find("</div>")
            .ok_or(anyhow!("aaaae"))?;

        println!("{} {}", name_start, name_len);

        println!(
            "{:?}",
            remaining_content
                .split_at(name_start)
                .1
                .split_at(name_len)
                .0
        );
        break;
    }

    Ok(html_content)
}

pub async fn fetch_spielplan() -> anyhow::Result<String> {
    let client = reqwest::ClientBuilder::new()
        .https_only(true)
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36")
        .use_rustls_tls()
        .cookie_store(true)
        .build()?;

    let mut headers = HeaderMap::new();
    headers.append(
        "Referer",
        HeaderValue::from_str("https://allianz-arena.com/de/spiele/spielplan/frauen")?,
    );
    headers.append(
        "Accept-Language",
        HeaderValue::from_str("de,en-US;q=0.7,en;q=0.3")?,
    );
    headers.append(
        "authorization",
        HeaderValue::from_str("X-Graphql-Operation: Web_MatchPlan")?,
    );
    headers.append(
        "Origin",
        HeaderValue::from_str("https://allianz-arena.com")?,
    );
    headers.append("DNT", HeaderValue::from_str("1")?);
    headers.append("Pragma", HeaderValue::from_str("no-cache")?);
    headers.append("Priority", HeaderValue::from_str("u=4")?);
    headers.append("Cache-Control", HeaderValue::from_str("no-cache")?);
    headers.append("Sec-Fetch-Dest", HeaderValue::from_str("empty")?);
    headers.append("Sec-Fetch-Mode", HeaderValue::from_str("cors")?);
    headers.append("Sec-Fetch-Site", HeaderValue::from_str("same-origin")?);

    let res = client.get("https://allianz-arena.com").send().await?;
    println!("{res:?}");
    println!("{:?}", client);

    headers.append("content-type", HeaderValue::from_str("application/json")?);

    let res = client
        .post("https://allianz-arena.com/graphql?client=fcbwebsite")
        .body(
            "{\"query\":\"
  __schema {
    queryType {
      fields {
        name
      }
    }
  }\"
}",
        )
        .headers(headers)
        .send()
        .await?;

    println!("{res:?}");
    println!("{res:?}");

    println!("Heeey");
    Ok("".to_string())
}
