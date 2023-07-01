# blog_os

https://os.phil-opp.com/ を見ながら実装(Thanks for writing and translating!)

## 変更点

- 一定期間停止する`sleep`関数を実装
  - タイマー割り込みがあるので、それを使って一定回数割り込まれるまでスリープする非同期関数を実装した
- 上の`sleep`関数によりマルチタスクの`example_task`をスリープありの無限ループにし、`exsample_task`と`pritn_keypresses`の両方が実行されていることを分かりやすくした。

![os](https://github.com/near129/blog_os/assets/56579877/45378b6a-320d-4510-9095-d98634934d4d)
