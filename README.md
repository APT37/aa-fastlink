## Anna's Archive ~ Fast Download Link Fetcher

Share your Anna's Archive membership, without sharing your secret key!

### Environment Variables

| Variable           | Description                                                                      | Default     |
| ------------------ | -------------------------------------------------------------------------------- | ----------- |
| `AA_SECRET`        | Secret key, check `Account` page.                                                | none        |
| `AA_DOMAIN`        | Mirror domain, e.g. `annas-archive.org`                                          | none        |
| `AA_BIND_IP`       | IP address to listen on.                                                         | `127.0.0.1` |
| `AA_BIND_PORT`     | Port to listen on.                                                               | `3030`      |
| `AA_DEBUG_LOGGING` | Enable debug logging. __CAREFUL: Secret key is logged when printing responses!__ | `false`     |

### Web Interface

Simple - but functional. Paste a book URL or MD5 hash and hit the `Download` button. The backend will fetch the fast download URL from the AA fast download API and redirect you to it. Enjoy reading. 📚