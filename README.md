## Anna's Archive - Fast Download Link Fetcher

Share your Anna's Archive membership, but without leaking your secret key!

### Environment Variables

- `AA_SECRET` Your secret key, found on the `Account` page.
- `AA_DOMAIN` The mirror domain, e.g. `annas-archive.org`
- `AA_BIND_IP` IP address to listen on. Default is `127.0.0.1`
- `AA_BIND_PORT` Port to listen on. Default is `3030`

### Interface

A simple and ugly interface, just paste a URL or MD5 hash of a book and hit the `Download` button. The backend will receive the fast download URL from the AA fast download API and redirect you to it. Easy! Enjoy reading. 📚