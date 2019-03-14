


//         fn convert_request<A: Serialize>(
//             request: &Request<Option<A>>,
//         ) -> Result<web_sys::Request, JsValue> {
//             let mut init = RequestInit::new();
//             init.method(request.method().as_str());
//             init.mode(RequestMode::Cors);

//             let headers = Headers::new()?;
//             for (k, v) in request.headers().iter() {
//                 let key = k.as_str();
//                 let val = v.to_str().unwrap(); // TODO don't panic
//                 headers.append(key, val)?;
//             }

//             if let Some(body) = request.body() {
//                 let value = JsValue::from_serde(body).unwrap(); // TODO don't panic
//                 init.body(Some(&value));
//             }

//             let uri = request.uri().to_string();
//             web_sys::Request::new_with_str_and_init(&uri, &init)
//         }

//         fn convert_response<B>(
//             response: web_sys::Response,
//             body: Option<B>,
//         ) -> Result<Response<Option<B>>, JsValue> {
//             Response::builder().status(response.status())
//         }

// pub fn fetch_json<T>(request: &web_sys::Request) -> impl Future<Item = T, Error = JsValue>
// where
//     for<'de> T: Deserialize<'de>,
// {
//     let window = web_sys::window().expect("should have a Window");
//     // let web_request = convert_request(request).unwrap(); // TODO don't panic
//     let request_promise = window.fetch_with_request(request);
//     JsFuture::from(request_promise)
//         .and_then(|resp_value| {
//             let resp: web_sys::Response = resp_value.dyn_into().unwrap(); // TODO don't panic
//             resp.json()
//         })
//         .and_then(JsFuture::from)
//         .and_then(|json| {
//             let data: T = json.into_serde().unwrap(); // TODO don't panic
//             future::ok(data)
//         })
// }

// pub fn fetch_blob(
//     request: web_sys::Request,
// ) -> impl Future<Item = web_sys::Blob, Error = JsValue> {
//     let window = web_sys::window().expect("should have a Window");
//     let request_promise = window.fetch_with_request(&request);
//     JsFuture::from(request_promise)
//         .and_then(|resp_value| {
//             let resp: web_sys::Response = resp_value.dyn_into().unwrap(); // TODO don't panic
//             resp.blob()
//         })
//         .and_then(JsFuture::from)
//         .map(|value| value.dyn_into::<web_sys::Blob>().unwrap()) // TODO don't panic
// }