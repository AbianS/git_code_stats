# Git Code Stats

![PowerShell Rust Project](./assets/git-stats.jpg)

Git Code Stats is a command-line application written in Rust that collects code statistics from Git repositories and creates a pie chart that displays the distribution of lines inserted by author.


## Usage

1. Download the executable from the [releases page](https://github.com/AbianS/git_code_stats/releases/tag/V1.0.0)
2. Add the executable to your PATH
3. with the terminal, navigate to the folder where the repository is located
4. Run the command `git-stats`

## Use of Multi-Threading for Better Performance

Git Code Stats utilizes multiple threads (multi-threading) to enhance performance when collecting code statistics from Git repositories. This technique allows processing multiple authors simultaneously, significantly speeding up the retrieval of statistics in repositories with a large number of authors or changes.

### Advantages of Multi-Threading

- **Greater Efficiency:** The use of multiple threads enables the full utilization of CPU processing capacity, resulting in faster and more efficient statistics retrieval.

- **Parallelization:** Each author is processed in a separate thread, enabling task parallelization and resource optimization.

- **Reduced Execution Time:** With multi-threading, the application can process multiple authors simultaneously, significantly reducing the total execution time, especially in large repositories.