# Gust
A charting library for rust!

[![Build Status](https://travis-ci.org/saresend/Gust.svg?branch=master)](https://travis-ci.org/saresend/Gust.svg?branch=master)


**Disclaimer**
This is still very much a work in progress! APIs are very unstable and subject to change. Contributions and suggestions are welcomed and greatly appreciated! 

---
<p align="center">
<img src=https://d30y9cdsu7xlg0.cloudfront.net/png/43264-200.png>
</p>

## What is Gust ##

Gust, is small a charting crate to make it really easy to build simple interactive data visualizations in rust. It also serves as a partial [Vega](http://vega.github.io/) implementation that will (hopefully) become more complete over time.

---

Gust allows you to render the actual visualizations themselves using D3.js, (meaning they're interactive!) as well as providing the flexibility to directly render the underlying JSON specification for Vega. 


### Currently Implementations ### 

Currently, Gust supports only 3 of the various charts that possible:

1. Bar Charts
2. Stacked Bar Charts
3. Line Charts

More will be coming soon! If you're interested in contributing your own, just read the [contributions](./CONTRIBUTING.md) page. Cheers!



## [Why did you do this in rust?](https://transitiontech.ca/random/RIIR) ## 

## Installation ## 
` gust = "0.1.1" `



## Samples and Sample Usage ## 

---

### Sample Bar Chart ###
```rust
     let mut b = BarChart::new();
        let v = ["A", "B", "C", "D", "E", "F", "G", "H", "I", "J"];
        for i in 0..10 {
            b.add_data(v[i].to_string(), (i * i * i) as i32);
        }
        b.save_to_file().unwrap();
```
### Result: **bar_chart.html** ###
![bar chart](./assets/bar_chart.png)

### Another Quick example ### 
```rust
let mut b = StackedBarChart::new();
        for i in 0..10 {
            b.add_data(i, i * i, 1);
            b.add_data(i, i + i, 0);
        }
        b.save_to_file().unwrap();
```
### Result: ### 
![stacked bar chart](./assets/stacked_bar.png)



## Additional Docs ## 
https://docs.rs/gust/0.1.1/gust/


## Special Mentions ## 
The rendering is all handled by Vega: https://vega.github.io/