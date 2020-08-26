#[allow(non_snake_case)]
use std::time::Instant;


const LIMIT: usize = 10000;

#[test]
fn test_get() {
    let response = crate::tcp::get("raw.githubusercontent.com", "/fatalcenturion/Curio/master/README.md");
    assert_eq!(response.status.unwrap(), 301);
}

#[test]
fn test_request_builder() {
    let mut request = crate::structs::Request::get("https://raw.githubusercontent.com/fatalcenturion/Curio/master/README.md");
    request.set_header("header", "true");
    assert_eq!(request.header_count, 1);
}

#[test]
fn test_tls_get() {
    let mut response = crate::structs::Request::get("https://raw.githubusercontent.com/fatalcenturion/Curio/master/README.md").send().unwrap();
    assert_eq!(response.status.unwrap(), 200);
}

#[test]
fn test_request_builder_get() {
    let response = crate::structs::Request::get("https://raw.githubusercontent.com/fatalcenturion/Curio/master/README.md").send().unwrap();
    assert_eq!(response.status.unwrap(), 200);
}

#[test]
fn bench_response_parsing() {
    bench("parse response", LIMIT, || {
        let header_line: Vec<String> = vec!["HTTP/1.1 301 TLS Redirect\r\n".to_string()];
        let test_body: Vec<String> = vec![
            "HTTP/1.1 301 TLS Redirect\r\n".to_string(),
            "Date: Fri, 21 Aug 2020 17:42:29 GMT\r\n".to_string(),
            "Content-Type: application/json; charset=utf-8\r\n".to_string(),
            "Connection: keep-alive\r\n".to_string(),
            "Set-Cookie: __cfduid=d1cd636ec4303be8a4ac9d8d01f93e1e71598031749; expires=Sun, 20-Sep-20 17:42:29 GMT; path=/; domain=.typicode.com; HttpOnly; SameSite=Lax\r\n".to_string(),
            "X-Powered-By: Express\r\n".to_string(),
            "X-Ratelimit-Limit: 1000\r\n".to_string(),
            "X-Ratelimit-Remaining: 999\r\n".to_string(),
            "X-Ratelimit-Reset: 1597842544\r\n".to_string(),
            "Vary: Origin, Accept-Encoding\r\n".to_string(),
            "Access-Control-Allow-Credentials: true\r\n".to_string(),
            "Cache-Control: max-age=43200\r\n".to_string(),
            "Pragma: no-cache\r\n".to_string(),
            "Expires: -1\r\n".to_string(),
            "X-Content-Type-Options: nosniff\r\n".to_string(),
            "Etag: W/\"5ef7-4Ad6/n39KWY9q6Ykm/ULNQ2F5IM\"\r\n".to_string(),
            "Via: 1.1 vegur\r\n".to_string(),
            "CF-Cache-Status: HIT\r\n".to_string(),
            "Age: 10212\r\n".to_string(),
            "cf-request-id: 04b3b67aed0000e608b91e0200000001\r\n".to_string(),
            "Server: cloudflare\r\n".to_string(),
            "CF-RAY: 5c6626a4ad9ae608-LHR".to_string()
        ];
        let response = crate::structs::Response::new(test_body.join(""), header_line);
    })
}

#[test]
fn bench_cookie_parsing() {
    let cookie = "Set-Cookie: has_recent_activity=1; path=/; expires=Fri, 21 Aug 2020 21:11:53 GMT; secure; HttpOnly; SameSite=Lax";
    bench("parse cookie", LIMIT, || {
        let cookie = crate::utils::parse_cookie(cookie.to_string());
    })
}

#[test]
fn bench_header_parsing() {
    let header = "Date: Fri, 21 Aug 2020 17:42:29 GMT";
    bench("parse header", LIMIT, || {
        let header = crate::utils::parse_header(header.to_string());
    })
}

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

