use warp::reply::{Reply, html};

pub(super) fn render_form() -> impl Reply {
    html(
        r#"
        <html>
            <head>
                <title>Fast Download</title>
            </head>
            <body>
                <form action="/dl" method="post">
                    <label for="link">Enter eBook URL or MD5: </label>
                    <input type="text" id="link" name="link" required>
                    <input type="submit" value="Download">
                </form>
            </body>
        </html>
        "#,
    )
}
