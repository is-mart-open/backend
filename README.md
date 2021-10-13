# is-mart-open-api

[API] 오늘 대형마트 영업하나요?

## REST API [WIP]

### 마트 검색

- URL

  `GET /search/:mart/:keyword`

- URL Params
  
  - `mart`: 마트 종류 (`emart`, `homeplus`, `costco`)
  - `keyword`: 검색할 점포 이름

- Success Response

  ```json
    {
        "status": "OK",
        "name": "이마트 가든5점",
        "start_time": "10:00",
        "end_time": "22:00",
        "next_holiday": "20211024"
    }
  ```