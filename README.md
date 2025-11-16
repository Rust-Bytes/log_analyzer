# Log Analyzer

Rust Bytes Issue 93 Challenge template


### Task

You are given a system log file named Mac_2k.log, located in the src directory.

The file contains real macOS-style log entries that follow this general pattern:

```
<Month> <Day> <Time> <Hostname> <Process>[PID]: <Message>
```
Write a Rust program that reads and parses the file Mac_2k.log. Each line should be parsed into:

- timestamp - e.g., Jul 1 09:00:55

- hostname - e.g., calvisitor-10-105-160-95

- process name - e.g., kernel, com.apple.cts, configd, etc.

- process ID (PID) - e.g., [0], [43]

- log message - everything after the colon

### What You Need to Produce

- Per-process stats: Count how many log entries each process generated (process name only, ignore PID).

- Per-hostname stats: Count how many entries came from each hostname.

- Global stats:

  - Most frequent process
  
  - Most frequent hostname
  - Optional: Most common keywords in log messages (lowercased, tokenized, stopwords removed)

Write all results to a **summary.json** file using the structure below:


<img width="1882" height="898" alt="Log_Analyzer_Rust_Bytes_Challenge" src="https://github.com/user-attachments/assets/b17eb89c-b2af-4d8e-ac67-a7363e96e986" />


You can start by cloning the example repo on [GitHub for reference](https://github.com/Rust-Bytes/log_analyzer).

