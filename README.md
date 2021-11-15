# is-mart-open/backend

[API] 오늘 마트 영업하나요?

## REST API [WIP]

### :warning: 주의

계속 업데이트 중인 문서입니다. 변동 사항이 있을 수 있습니다.

### 지점 목록 조회

- URL

  `GET /mart/list`

- Success Response

  ```json
  {
    "result": [
      "이마트 가든5점",
      "이마트 가양점",
      "이마트 검단점"
    ]
  }
  ```

### 지점 조회

- URL

  `GET /mart/:name`

- URL Params

  - `name`: 점포 이름

- Success Response

  ```json
  {
    "base_date": "2021-10-28",
    "name": "이마트 경산점",
    "open_time": "10:00",
    "close_time": "23:00",
    "next_holiday": "2021-10-27",
    "distance": null
  }
  ```

- Error Response

  ```json
  { "error": "검색 결과가 없어요" }
  ```

### 가까운 지점 조회

가까운 지점의 반경은 `10km` 로 제한되어있으며 단위는 `m` 입니다.

- URL

  `GET /mart/from-location?lat=:lat&lon=:lon`

- URL Params
  
  - `lat`: 기준 위치의 위도
  - `lon`: 기준 위치의 경도

- Success Response

  ```json
  {
    "result": [
      {
        "base_date": "2021-10-28",
        "name": "이마트 경산점",
        "open_time": "10:00",
        "close_time": "23:00",
        "next_holiday": "2021-10-27",
        "distance": "1254"
      }
    ]
  }
  ```

- Error Response

  ```json
  { "error": "반경 10km 이내 가까운 마트가 없어요" }
  ```
