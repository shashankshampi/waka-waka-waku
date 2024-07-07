<a name="readme-top"></a>

<!-- PROJECT LOGO -->
<br />
<div align="center">
  <a href="https://github.com/shashankshampi/waka-waka-waku">
    <img src="files/waku.svg" alt="Logo" width="140" height="100">
  </a>

<h3 align="center">waka waka waku</h3>

  <p align="center">
    An awesome DAY to jumpstart your day!
    <br />
    <a href="https://github.com/shashankshampi/waka-waka-waku"><strong>Explore README »</strong></a>
    <br />
    <br />
  </p>
</div>



<!-- TABLE OF CONTENTS -->
<details open>
  <summary>Table of Contents</summary>
  <br>
  <ol>
    <li>
      <a href="#Overview">About The Project</a>
      <ul>
        <li><a href="#Features">Features</a></li>
        <li><a href="#Built-With">Built With</a></li>
      </ul>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#prerequisites">Prerequisites</a></li>
        <li><a href="#Configuration">Configuration</a></li>
      </ul>
    </li>
    <li><a href="#usage">Usage</a></li>
    <li><a href="#Building-the-Application">Building the Application</a></li>
    <li><a href="#Running-the-Application">Running the Application</a></li>
  <li><a href="#Command-Line-Flags">Command Line Flags</a></li>
  <li><a href="#Contributing">Contributing</a></li>
    <li><a href="#contact">Contact</a></li>
  </ol>
</details>


<!-- ABOUT THE PROJECT -->

## Overview
waka-waka-waku serves as the testing hub for waku. This rust-based repository excels in conducting functional, integration, and regression tests. Furthermore, its versatility extends to scalability, seamlessly integrating with the Scala-based Gatling repository for rigorous performance testing.

Framework structure
```html
waka-waka-waku/
├── Cargo.toml
├── .github/
│   ├── workflows/
│       └── pipeline.yml
├── src/
│   ├── lib.rs
│   └── main.rs
└── tests/
    └── integration_tests.rs
```

## Features

- **Versatile Testing Capabilities**: waka-waka-waku is basic test framework
- **Seamless Integration**: Used Action pipeline to start docker container for test setup to execute functional, integration and e2e testing.


<p align="right">(<a href="#readme-top">back to top</a>)</p>

[//]: # ([<img alt="flow.jpeg" src="flow.png"/>]&#40;https://github.com/shashankshampi/waka-waka-waku&#41;)

### Built With

waka-waka-waku is build on the following Tech Stack.

* [![Rust](https://img.shields.io/badge/Rust-1.66%2B-DEA584)][rust-url]
* [![GitHub Actions](https://img.shields.io/badge/GitHub%20Actions-Reference-2088FF)][github-actions-url]
* [![Waku](https://img.shields.io/badge/Waku-Reference-brightgreen)][waku-url]

[rust-url]: https://www.rust-lang.org/
[github-actions-url]: https://docs.github.com/en/actions
[waku-url]: https://docs.waku.org/guides/nwaku/run-docker/

<p align="right">(<a href="#readme-top">back to top</a>)</p>


<!-- GETTING STARTED -->

## Getting Started

To start with this project you need to follow the steps as below.

### Prerequisites

Before you begin, ensure you have the following installed:
- Rust 1.66+
- Docker

### Library used
 ```
 - reqwest
 - serde
 - serde_json
 - tokio
 - urlencoding
 - serial_test
 ```

## Configuration

1. Start a Waku Node:
```
docker run -i -t \
    -p 21161:21161 \
    -p 21162:21162 \
    -p 21163:21163 \
    -p 21164:21164 \
    -p 21165:21165 \
    wakuorg/nwaku:v0.24.0 \
    --listen-address=0.0.0.0 \
    --rest=true \
    --rest-admin=true \
--websocket-support=true \
--log-level=TRACE \
--rest-relay-cache-capacity=100 \
--websocket-port=21163 \
--rest-port=21161 \
--tcp-port=21162 \
--discv5-udp-port=21164 \
--rest-address=0.0.0.0 \
--nat=extip:172.18.111.226 \
--peer-exchange=true \
--discv5-discovery=true \
--relay=true
```
2. Verify Node Information:

```
curl --location 'localhost:21161/debug/v1/info'
```
3. Start a Second Node:

```
docker run -i -t \
    -p 21261:21161 \
    -p 21262:21162 \
    -p 21263:21163 \
    -p 21264:21164 \
    -p 21265:21165 \
    wakuorg/nwaku:v0.24.0 \
    --listen-address=0.0.0.0 \
    --rest=true \
    --rest-admin=true \
    --websocket-support=true \
    --log-level=TRACE \
    --rest-relay-cache-capacity=100 \
    --websocket-port=21263 \
    --rest-port=21161 \
    --tcp-port=21262 \
    --discv5-udp-port=21264 \
    --rest-address=0.0.0.0 \
    --nat=extip:172.18.111.227 \
    --peer-exchange=true \
    --discv5-discovery=true \
    --relay=true \
    --discv5-bootstrap-node=<enrUri_stored_from_step 2>

```
4. Create a Docker Network:
```
 docker network create --driver bridge --subnet 172.18.0.0/16 --gateway 172.18.0.1 waku
```
5. Attach Node Containers to the Network:

```
docker network connect --ip 172.18.111.226 waku <container_id_node1>
docker network connect --ip 172.18.111.227 waku <container_id_node2>
```
6. Verify Auto-connection:

Verification with waku api
```
curl --location 'localhost:21161/admin/v1/peers'
```

Verification with docker command:
```sh
docker network inspect waku
```

Sample of response:
```
[
    {
        "Name": "waku",
        "Id": "aca133f767b5efcadc670087f75a58a2bf27ae1befb80f96b0c79f41077e994a",
        "Created": "2024-07-06T14:33:33.56994768Z",
        "Scope": "local",
        "Driver": "bridge",
        "EnableIPv6": false,
        "IPAM": {
            "Driver": "default",
            "Options": {},
            "Config": [
                {
                    "Subnet": "172.18.0.0/16",
                    "Gateway": "172.18.0.1"
                }
            ]
        },
        "Internal": false,
        "Attachable": false,
        "Ingress": false,
        "ConfigFrom": {
            "Network": ""
        },
        "ConfigOnly": false,
        "Containers": {
            "98a31ac476d37fba4c9957cd6788f039e5d23ad6a236962c568c421a1016409d": {
                "Name": "clever_lewin",
                "EndpointID": "f67e809af2ac26a7688da2fa393606c96e210a17a3db6d9bb576560f68b24b6b",
                "MacAddress": "02:42:ac:12:6f:e2",
                "IPv4Address": "172.18.111.226/16",
                "IPv6Address": ""
            },
            "b67189897b2e020217285ce178a03750e706238678309267dd21632b69028dd8": {
                "Name": "epic_liskov",
                "EndpointID": "47524b5cf378a3e22cd28a4e1ca2e42757aa82bad9caa281e62a74694ed42455",
                "MacAddress": "02:42:ac:12:6f:e3",
                "IPv4Address": "172.18.111.227/16",
                "IPv6Address": ""
            }
        },
        "Options": {},
        "Labels": {}
    }
]
```

Expect Response with connected as `true`
```
[
    {
        "multiaddr": "/ip4/172.18.111.227/tcp/21262/p2p/16Uiu2HAmGYaB9meUF5FjQ9P7vtAtX25Am2CV63R5vZ8V3jzGQMMB",
        "protocols": [
            {
                "protocol": "/vac/waku/relay/2.0.0",
                "connected": true
            }
        ]
    }
]
```

<!-- USAGE EXAMPLES -->

## Usage

Clone the repository locally and execute the respective test xml file located in dir suiteXMLs.
<div align="center">

[//]: # (<img src="swagger.png" alt="Logo" >)
</div>

<p align="right">(<a href="#readme-top">back to top</a>)</p>

## Building the Application

To build your application with Github Action and run your test in action runner you can raise your PR against the main and can see the execution.

1. Action URL:

```html
https://github.com/shashankshampi/waka-waka-waku/actions
```

2. Steps to start waku docker nodes with pipeline is written over:

```html
https://github.com/shashankshampi/waka-waka-waku/blob/main/.github/workflows/pipeline.yml
```

3. Check the Test result in action pipeline in step `Build and Test Rust Code`

Sample Run:

```html
https://github.com/shashankshampi/waka-waka-waku/actions/runs/9829568324/job/27134743168
```
<img src="files/img.png" alt="sample_run" width="400" height="200">


<p align="right">(<a href="#readme-top">back to top</a>)</p>

## Running the Application

You can run the application directly with the following command:

1. Build Your Application
```sh
cargo build --release
```

2. Run Your Application
```sh
cargo run --release
```
3. Run Test in sequential order

```sh
cargo test --test integration_tests -- --test-threads=1
```

<p align="right">(<a href="#readme-top">back to top</a>)</p>

## Postman collection of waku assignment APIs

[waku.postman_collection.json](files%2Fwaku.postman_collection.json)

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- CONTRIBUTING -->

## Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any
contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also
simply open an issue with the tag "enhancement".
Don't forget to give the project a star! Thanks again!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

<p align="right">(<a href="#readme-top">back to top</a>)</p>


<!-- CONTACT -->

## Contact

Developer - [@shashank sanket](shashank.sanket1995@gmail.com)

<p align="right">(<a href="#readme-top">back to top</a>)</p>


<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->
