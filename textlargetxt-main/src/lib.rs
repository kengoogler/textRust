use memmap2::MmapMut;
use rand::{ distributions::Alphanumeric, Rng };
use std::fs::OpenOptions;
use std::sync::{ Arc, Mutex };
use std::time::Duration;
use std::time::Instant;
use tokio::task;
pub async fn calculate(num: String) {
    let start_time = Instant::now(); // 獲取開始時間

    let input = num;
    let file_size_mb: u64 = input.trim().parse().expect("請輸入有效的數字");

    // 使用創建並設置文件大小函數
    let mmap = create_and_set_file_size(file_size_mb).await;

    // 内存映射文件
    // 内存映射文件
    // let mmap = unsafe { MmapMut::map_mut(&file).expect("無法將檔案映射到記憶體") };
    // let mmap = Arc::new(Mutex::new(mmap));

    // 使用多线程批量生成数据
    let num_threads = 16; // 增加線程數量
    let chunk_size = (file_size_mb as usize) / num_threads;
    let handles: Vec<_> = (0..num_threads)
        .map(|i| {
            let mmap = Arc::clone(&mmap);
            let start = i * chunk_size;
            let end = if i == num_threads - 1 {
                file_size_mb as usize
            } else {
                (i + 1) * chunk_size
            };

            task::spawn(async move {
                let mut rng = rand::thread_rng();
                let mut offset = start;
                let mut buffer: Vec<u8> = Vec::with_capacity(chunk_size);

                // 按照 30 行的规律填充数据
                while offset < end {
                    let line: String = (0..30).map(|_| rng.sample(Alphanumeric) as char).collect();
                    buffer.extend_from_slice(line.as_bytes());
                    buffer.push(b'\n');
                    offset += 31; // 每行 30 字符，1 个换行符
                }

                // 确保不超出目标范围
                let buffer_size = buffer.len();
                let chunk_slice = if buffer_size > end - start {
                    &buffer[..end - start]
                } else {
                    &buffer[..buffer_size]
                };

                // 通过锁定的映射文件写入数据块
                let mut mmap = mmap.lock().unwrap();
                mmap[start..start + chunk_slice.len()].copy_from_slice(chunk_slice);
            })
        })
        .collect();

    // 等待所有线程完成
    for handle in handles {
        handle.await.expect("Thread panicked");
    }

    let duration = start_time.elapsed(); // 計算執行時間

    println!("函數執行時間：{:?}", duration); // 輸出執行時間
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::fs::remove_file;
    #[tokio::test]
    async fn test_calculate() {
        let num = "10";
        // 創建一個 1MB 的檔案
        let _c = calculate(num.to_string()).await;

        // 檢查檔案是否存在
        assert!(std::path::Path::new(&(num.to_owned() + ".txt")).exists());

        // 清理測試檔案
        remove_file(&(num.to_owned() + ".txt").to_string()).await.expect(
            "Failed to delete '1'.txt"
        );
    }
}
async fn create_and_set_file_size(file_size_mb: u64) -> Arc<Mutex<MmapMut>> {
    let file_size = file_size_mb * 1024 * 1024;

    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(format!("{}.txt", file_size_mb))
        .expect("無法開啟檔案");
    file.set_len(file_size).expect("無法設定檔案長度");

    let mmap = unsafe { MmapMut::map_mut(&file).expect("無法將檔案映射到記憶體") };
    Arc::new(Mutex::new(mmap))
}
