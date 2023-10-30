# primebench

`primebench` is a Rust project that benchmarks prime number calculations for various settings, such as the range of numbers to calculate primes for and the number of threads used for parallel processing.

## How to Use

1. Clone the repository:

    ```bash
    git clone https://github.com/Staninna/  primebench.git
    ```

2. Navigate to the project directory:

    ```bash
    cd primebench
    ```

3. Set environment variables:

    - `START_N`: The starting number for prime  calculations.
    - `MAX_N`: The maximum number for prime calculations.
    - `MIN_THREADS`: The minimum number of threads to use.
    - `MAX_THREADS`: The maximum number of threads to use.
    - `TRIES`: The number of benchmarking tries.

    You can set these environment variables in a `.env` file or directly in your shell.

4. Build and run the project:

    ```bash
    cargo run --release
    ```

    The project will execute multiple benchmarking  tries with varying thread counts and output the  best configuration.

## Results

The project will provide information about the best-performing configuration, including the number of threads, execution time, and other details.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
