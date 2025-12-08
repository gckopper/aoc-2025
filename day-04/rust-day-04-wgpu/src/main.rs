use std::{env, fs, io::Read};

use pollster::FutureExt as _;
use wgpu::util::DeviceExt;

fn main() {
    let args = env::args()
        .nth(1)
        .expect("Erm... we need a file as input... erm...");
    let input = fs::read_to_string(args).expect("Erm... Unable to read file... erm");
    
    let line_size = input.find('\n').expect("no \\n in input");
    let input: Vec<u32> = input.chars().filter(|c| *c != '\n').map(|c| c as u32).collect();
    let column_size = input.len() / line_size;
    println!("line: {line_size} column_size: {column_size}");

    let code = async {
        let instance = wgpu::Instance::new(&Default::default());
        let adapter = instance.request_adapter(&Default::default()).await.unwrap();
        let (device, queue) = adapter.request_device(&Default::default()).await.unwrap();
        
        part1(&device, &input, &queue, column_size, line_size).await;
        part2(&device, &input, &queue, column_size, line_size).await;
    };
    let _ = code.block_on();
}

async fn part1(device: &wgpu::Device, input: &Vec<u32>, queue: &wgpu::Queue, column_size: usize, line_size: usize) {
        let shader = device.create_shader_module(wgpu::include_wgsl!("./part1.wgsl"));

        let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("Part 1 compute shader"),
            layout: None,
            module: &shader,
            entry_point: None,
            compilation_options: Default::default(),
            cache: Default::default()
        });
        
        let input_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("input buffer"),
            contents: bytemuck::cast_slice(&input),
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::STORAGE,
        });

        let output_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("output buffer"),
            size: input_buffer.size(),
            usage: wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::STORAGE,
            mapped_at_creation: false,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("bind group"),
            layout: &pipeline.get_bind_group_layout(0),
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: input_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: output_buffer.as_entire_binding(),
                },
            ],
        });

        let temp_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("temp buffer"),
            size: input_buffer.size(),
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        {
            let mut pass = encoder.begin_compute_pass(&Default::default());
            pass.set_pipeline(&pipeline);
            pass.set_bind_group(0, &bind_group, &[]);
            pass.dispatch_workgroups(line_size as u32, column_size as u32, 1);
        }

        encoder.copy_buffer_to_buffer(&output_buffer, 0, &temp_buffer, 0, output_buffer.size());

        queue.submit([encoder.finish()]);

        let total: u64;

        {
            // The mapping process is async, so we'll need to create a channel to get
            // the success flag for our mapping
            let (tx, rx) = flume::unbounded();

            // We send the success or failure of our mapping via a callback
            temp_buffer.map_async(wgpu::MapMode::Read, .., move |result| tx.send(result).unwrap());

            // The callback we submitted to map async will only get called after the
            // device is polled or the queue submitted
            device.poll(wgpu::PollType::wait_indefinitely()).expect("failed while polling gpu device");

            // We check if the mapping was successful here
            rx.recv().expect("failed to recv bytes from channel").expect("error mapping temp buffer");

            // We then get the bytes that were stored in the buffer
            let output_data = temp_buffer.get_mapped_range(..);

            // Now we have the data on the CPU we can do what ever we want to with it
            //assert_eq!(&input, bytemuck::cast_slice(&output_data));
            //println!("{:?}", bytemuck::cast_slice::<u8, u32>(&output_data));

            total = output_data.bytes().map(|b| b.unwrap() as u64).sum::<u64>() / 64;
        }

        println!("final: {total}");

        // We need to unmap the buffer to be able to use it again
        temp_buffer.unmap();
}

async fn part2(device: &wgpu::Device, input: &Vec<u32>, queue: &wgpu::Queue, column_size: usize, line_size: usize) {
        let shader = device.create_shader_module(wgpu::include_wgsl!("./part2.wgsl"));

        let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("Part 2 compute shader"),
            layout: None,
            module: &shader,
            entry_point: None,
            compilation_options: Default::default(),
            cache: Default::default()
        });

        let input_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("input buffer part 2"),
            contents: bytemuck::cast_slice(&input),
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::STORAGE,
        });

        let output_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("output buffer part 2"),
            size: input_buffer.size(),
            usage: wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::STORAGE,
            mapped_at_creation: false,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("bind group part 2"),
            layout: &pipeline.get_bind_group_layout(0),
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: input_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: output_buffer.as_entire_binding(),
                },
            ],
        });

        let temp_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("temp buffer part 2"),
            size: input_buffer.size(),
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder part 2"),
        });

        for _ in 0..1000 {
            {
                let mut pass = encoder.begin_compute_pass(&Default::default());
                pass.set_pipeline(&pipeline);
                pass.set_bind_group(0, &bind_group, &[]);
                pass.dispatch_workgroups(line_size as u32, column_size as u32, 1);
            }

            encoder.copy_buffer_to_buffer(&output_buffer, 0, &input_buffer, 0, output_buffer.size());
        }

        encoder.copy_buffer_to_buffer(&output_buffer, 0, &temp_buffer, 0, output_buffer.size());

        queue.submit([encoder.finish()]);

        let total: usize;

        {
            // The mapping process is async, so we'll need to create a channel to get
            // the success flag for our mapping
            let (tx, rx) = flume::unbounded();

            // We send the success or failure of our mapping via a callback
            temp_buffer.map_async(wgpu::MapMode::Read, .., move |result| tx.send(result).unwrap());

            // The callback we submitted to map async will only get called after the
            // device is polled or the queue submitted
            device.poll(wgpu::PollType::wait_indefinitely()).expect("failed while polling gpu device");

            // We check if the mapping was successful here
            rx.recv().expect("failed to recv bytes from channel").expect("error mapping temp buffer");

            // We then get the bytes that were stored in the buffer
            let output_data = temp_buffer.get_mapped_range(..);

            let output = bytemuck::cast_slice::<u8, u32>(&output_data);

            // Now we have the data on the CPU we can do what ever we want to with it
            //assert_eq!(&input, bytemuck::cast_slice(&output_data));
            //println!("{:?}", output);

            total = output.iter().filter(|b| **b == 0).count();
        }

        println!("final: {total}");

        // We need to unmap the buffer to be able to use it again
        temp_buffer.unmap();
}
