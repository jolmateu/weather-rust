use actix_web::{web, App, HttpServer, Responder};
use actix_files::Files;
use serde::Deserialize;

#[derive(Deserialize)]
struct WeatherResponse {
    main: WeatherMain,
    weather: Vec<WeatherCondition>,
}

#[derive(Deserialize)]
struct WeatherMain {
    temp: f32,
}

#[derive(Deserialize)]
struct WeatherCondition {
    main: String,
    icon: String,
}

async fn index() -> impl Responder {
    let html = r#"
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="UTF-8">
                <meta name="viewport" content="width=device-width, initial-scale=1.0">
                <link rel="stylesheet" href="/static/style.css">
                <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/5.15.4/css/all.min.css" />
                <link rel="preconnect" href="https://fonts.googleapis.com">
                <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
                <link href="https://fonts.googleapis.com/css2?family=Montserrat&display=swap" rel="stylesheet">
                <title>Weather Forecast</title>
            </head>
            <body>
                <div class="weather-forecast">
                    <div class="weather-forecast-form">
                        <form action="/weather" method="post">
                            <label for="city">Choose a city:</label>
                            <select name="city" id="city">
                                <option value="Chicago">Chicago</option>
                                <option value="Hollywood">Hollywood</option>
                                <option value="Miami">Miami</option>
                                <option value="New York">New York</option>
                                <option value="Orlando">Orlando</option>
                                <option value="Salt Lake City">Salt Lake City</option>
                                <option value="Texas">Texas</option>
                                <option value="Washington">Washington</option>
                            </select>
                            <br><br>
                            <input type="submit" value="Submit">
                        </form>
                    </div>
                    <div class="weather-cards" id="weather-cards">
                        <!-- Weather cards will be dynamically populated here -->
                    </div>
                </div>
            </body>
            <script>
                // You can add JavaScript here if needed
            </script>
        </html>
    "#;

    actix_web::HttpResponse::Ok().content_type("text/html").body(html)
}

async fn get_weather(form: web::Form<FormData>) -> impl Responder {
    // Perform API call to OpenWeatherMap using reqwest
    let api_key = "e58c9d2940678205482ada58cff75959";
    let city = form.city.clone();
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric",
        city, api_key
    );

    let response = reqwest::get(&url).await;

    if let Ok(res) = response {
        let weather_data: WeatherResponse = res.json().await.unwrap_or_else(|_| {
            WeatherResponse {
                main: WeatherMain { temp: 0.0 },
                weather: vec![WeatherCondition {
                    main: "Unknown".to_string(),
                    icon: "".to_string()
                }],
            }
        });

        // Logic to dynamically populate the weather cards
        let weather_html = format!(
            r#"
            <html lang="en">
            <head>
                <meta charset="UTF-8">
                <meta name="viewport" content="width=device-width, initial-scale=1.0">
                <link rel="stylesheet" href="/static/style.css">
                <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/5.15.4/css/all.min.css" />
                <link rel="preconnect" href="https://fonts.googleapis.com">
                <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
                <link href="https://fonts.googleapis.com/css2?family=Montserrat&display=swap" rel="stylesheet">
                <title>Weather Forecast</title>
            </head>
            <body>
                <div class="weather-card">
                    <h2>Actual Weather</h2>
                    <img src="/static/images/{}.png" alt="Weather Icon" width="100" height="100">
                    <p class="day">{} ({})</p>
                    <p class="temperature">{:.1}°C</p>
                </div>
            </body>
            "#,
            weather_data.weather[0].icon, city, weather_data.weather[0].main, weather_data.main.temp
        );

        // Return the dynamically generated weather cards
        actix_web::HttpResponse::Ok().content_type("text/html").body(weather_html)
    } else {
        // Handle error
        actix_web::HttpResponse::InternalServerError().body("Error fetching weather data")
    }
    
}

#[derive(Deserialize)]
struct FormData {
    city: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(Files::new("/static", "static").show_files_listing())
            .service(web::resource("/").route(web::get().to(index)))
            .service(web::resource("/weather").route(web::post().to(get_weather)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}