use std::time::{Duration, Instant};

use anyhow::{anyhow, Result};
use tokio_graceful_shutdown::wait_until_shutdown_started;

use log;

use tokio::io::{AsyncReadExt, AsyncWriteExt, ReadHalf, WriteHalf};
use tokio_serial::{self, SerialPortBuilderExt, SerialStream};

const BUF_SIZE: usize = 1024;

pub async fn reader(mut serial: ReadHalf<SerialStream>) -> Result<()> {
    let mut buf: [u8; BUF_SIZE] = [0; BUF_SIZE];

    let mut total: usize = 0;

    let mut next_expected = 0;

    let mut measure_start = Instant::now();
    let mut measure_num = 0;

    loop {
        tokio::task::yield_now().await;
        let num_read = serial.read(&mut buf).await?;
        total += num_read;
        measure_num += num_read;

        for val in buf[..num_read].iter() {
            anyhow::ensure!(val == &next_expected, "Invalid data received!");
            next_expected = next_expected.wrapping_add(7);
        }

        let now = Instant::now();
        let diff = now - measure_start;
        if diff > Duration::from_millis(1000) {
            log::info!("Read: {}/s, total: {}", measure_num, total);
            measure_start = now;
            measure_num = 0;
        }
    }
}

pub async fn writer(mut serial: WriteHalf<SerialStream>) -> Result<()> {
    let mut buf: [u8; BUF_SIZE] = [0; BUF_SIZE];

    let mut total: usize = 0;

    let mut measure_start = Instant::now();
    let mut measure_num = 0;

    let mut i = 0;
    loop {
        tokio::task::yield_now().await;
        for val in buf.iter_mut() {
            *val = i;
            i = i.wrapping_add(7);
        }

        //tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
        let num_written = serial.write(&buf).await?;
        total += num_written;
        measure_num += num_written;

        let now = Instant::now();
        let diff = now - measure_start;
        if diff > Duration::from_millis(1000) {
            log::info!("Written: {}/s, total: {}", measure_num, total);
            measure_start = now;
            measure_num = 0;
        }
    }
}

pub async fn run() -> Result<()> {
    let ports = tokio_serial::available_ports().expect("No ports found!");
    for p in &ports {
        log::info!("Opening port {} ...", p.port_name);
    }

    let port_info = ports.first().ok_or(anyhow!("No serial port found!"))?;

    let serial = tokio_serial::new(&port_info.port_name, 9600).open_native_async()?;
    let (serial_read, serial_write) = tokio::io::split(serial);

    tokio::select! {
        e = reader(serial_read) => e,
        e = writer(serial_write) => e,
        _ = wait_until_shutdown_started() => Ok(()),
    }
}
