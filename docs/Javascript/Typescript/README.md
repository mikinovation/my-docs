# Typescript

## 定数について

基本はユニオン型。順番に意味があるものは別途配列を作成

```ts
const STATUS = {
    UNSTARTED: 0,
    STARTED: 1,
    FINISHED: 2,
    DELIVERED: 3,
    REJECTED: 4,
    ACCEPTED: 5,
} as const

const STATUSES = [
    { name: 'Unstarted', value: STATUS.UNSTARTED },
    { name: 'Started', value: STATUS.STARTED },
    { name: 'Finished', value: STATUS.FINISHED },
    { name: 'Delivered', value: STATUS.DELIVERED },
    { name: 'Rejected', value: STATUS.REJECTED },
    { name: 'Accepted', value: STATUS.ACCEPTED },
] as const

export type Status = typeof STATUS[keyof typeof STATUS]
```
参考: [[JavaScript] オブジェクト/Mapのキーの列挙順は保証されるのか](https://qiita.com/anqooqie/items/ab3fed5d269d278cdd2b)