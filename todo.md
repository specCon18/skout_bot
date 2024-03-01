**Every 30 Mins call this query**

```bash
curl \
  'https://youtube.googleapis.com/youtube/v3/search?part=snippet&channelId=UC8ZFJetaO6NiRKERz6JXMcA&eventType=live&maxResults=5&type=video&key=[YOUR_API_KEY]' \
  --header 'Authorization: Bearer [YOUR_ACCESS_TOKEN]' \
  --header 'Accept: application/json' \
  --compressed

```
**to get this kind of output:**

```json
{
  "kind": "youtube#searchListResponse",
  "etag": "A-ZaAqcgivlZz9r3rz6M2CDBP90",
  "regionCode": "US",
  "pageInfo": {
    "totalResults": 1,
    "resultsPerPage": 1
  },
  "items": [
    {
      "kind": "youtube#searchResult",
      "etag": "7R3q8UXbufZW8xCyjZ4E8dpHzOY",
      "id": {
        "kind": "youtube#video",
        "videoId": "6HMKgfFgzDs"
      },
      "snippet": {
        "publishedAt": "2023-12-18T10:03:15Z",
        "channelId": "UC8ZFJetaO6NiRKERz6JXMcA",
        "title": "test",
        "description": "",
        "thumbnails": {
          "default": {
            "url": "https://i.ytimg.com/vi/6HMKgfFgzDs/default_live.jpg",
            "width": 120,
            "height": 90
          },
          "medium": {
            "url": "https://i.ytimg.com/vi/6HMKgfFgzDs/mqdefault_live.jpg",
            "width": 320,
            "height": 180
          },
          "high": {
            "url": "https://i.ytimg.com/vi/6HMKgfFgzDs/hqdefault_live.jpg",
            "width": 480,
            "height": 360
          }
        },
        "channelTitle": "ThatGeodeGamer",
        "liveBroadcastContent": "live",
        "publishTime": "2023-12-18T10:03:15Z"
      }
    }
  ]
}
```

** if a stream is live, notify the discord. **