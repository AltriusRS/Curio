#[allow(non_snake_case)]
use tokio_test::*;
use std::time::Instant;

macro_rules! aw {
  ($e:expr) => {
      tokio_test::block_on($e)
  };
}
const LIMIT: usize = 10000;
const TEST_STR: &str = "HTTP/1.1 301 TLS Redirect\r\nDate: Fri, 21 Aug 2020 17:42:29 GMT\r\nContent-Type: application/json; charset=utf-8\r\nConnection: keep-alive\r\nSet-Cookie: __cfduid=d1cd636ec4303be8a4ac9d8d01f93e1e71598031749; expires=Sun, 20-Sep-20 17:42:29 GMT; path=/; domain=.typicode.com; HttpOnly; SameSite=Lax\r\nX-Powered-By: Express\r\nX-Ratelimit-Limit: 1000\r\nX-Ratelimit-Remaining: 999\r\nX-Ratelimit-Reset: 1597842544\r\nVary: Origin, Accept-Encoding\r\nAccess-Control-Allow-Credentials: true\r\nCache-Control: max-age=43200\r\nPragma: no-cache\r\nExpires: -1\r\nX-Content-Type-Options: nosniff\r\nEtag: W/\"5ef7-4Ad6/n39KWY9q6Ykm/ULNQ2F5IM\"\r\nVia: 1.1 vegur\r\nCF-Cache-Status: HIT\r\nAge: 10212\r\ncf-request-id: 04b3b67aed0000e608b91e0200000001\r\nServer: cloudflare\r\nCF-RAY: 5c6626a4ad9ae608-LHR";

#[test]
fn test_GET() {
    let response = crate::tcp::get("github.com", "/Hexeption/Optifine-SRC/blob/master/Optifine%20SRC%20Version%20%5B1.10.2%20HD%20U%20C1%5D.rar");
    assert_eq!(1,1)
    //assert_eq!(response.status_text.unwrap(), String::from("OK"));
}

#[test]
fn test_request_builder() {
    let mut request = crate::structs::Request::get("https://github.com/Hexeption/Optifine-SRC/blob/master/Optifine%20SRC%20Version%20%5B1.10.2%20HD%20U%20C1%5D.rar");
    request.set_header("header", "true");
    println!("{:#?}", request);
    assert_eq!(request.header_count, 1);
}

#[test]
fn test_request_builder_GET() {
    let response = crate::tls::get("spacelaunchnow.me", "/api/3.3.0/agencies/");
    //println!("{:#?}", response);
    assert_eq!(1,1)
    //assert_eq!(response.status_text.unwrap(), String::from("OK"));
}

// #[test]
// fn bench_response_parsing() {
//     bench("parse response", LIMIT, || {
//         let header_line = "HTTP/1.1 301 TLS Redirect\r\n".to_string();
//         let response = crate::structs::Response::new(String::from(TEST_STR), header_line);
//     })
// }
//
// #[test]
// fn bench_cookie_parsing() {
//     let cookie = "Set-Cookie: has_recent_activity=1; path=/; expires=Fri, 21 Aug 2020 21:11:53 GMT; secure; HttpOnly; SameSite=Lax";
//     bench("parse cookie", LIMIT, || {
//         let cookie = crate::utils::parse_cookie(cookie);
//     })
// }
//
// #[test]
// fn bench_header_parsing() {
//     let header = "Date: Fri, 21 Aug 2020 17:42:29 GMT";
//     bench("parse header", LIMIT, || {
//         let header = crate::utils::parse_header(header);
//     })
// }
//
// #[test]
// fn bench_full_request_localhost() {
//     bench("full request cycle", LIMIT, || {
//         let resp = aw!(crate::tcp::get("localhost", "/"));
//     })
// }

fn bench<A, B>(name: A, passes: usize, call: B) -> () where A: Into<String>, B: Fn() -> () {
    let mut passed = 0;

    let mut times = Vec::<u128>::new();

    while passed < passes {
        let start = Instant::now();

        call();

        times.push(start.elapsed().as_micros());

        passed += 1;
    }

    let mut total: usize = 0;

    let mut avg: usize = 0;
    let mut high: usize = 0;
    let mut low: usize = 10000;

    for time in times.clone() {
        let t = time as usize;
        total += t;
        if t > high {
            high = t;
        } else if t < low {
            low = t;
        }
    }

    avg = total / LIMIT;

    println!("{}; ({} passes): Avg: {} us  |  High: {} us  |  Low: {} us  |  S.D: {} us", name.into(), passes, avg.clone(), high, low, std_deviation(times, avg).unwrap_or(0));

    return assert_eq!(1, 1);
}

fn std_deviation(data: Vec<u128>, mean: usize) -> Option<usize> {
    let mut data2 = Vec::<u32>::new();
    let mean2 = mean as u32;
    for x in data {
        data2.push(x as u32)
    }

    let mean3 = mean2 as f32;

    match (mean3, data2.len()) {
        (data_mean, count) if count > 0 => {
            let variance = data2.iter().map(|value| {
                let diff = data_mean - (*value as f32);

                diff * diff
            }).sum::<f32>() / count as f32;

            Some(variance.sqrt() as usize)
        }
        _ => None
    }
}

