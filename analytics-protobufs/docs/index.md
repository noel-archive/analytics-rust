---
title: Protocol Buffers for Analytics Engine
description: Main page about the Protocol Buffers for Noelware Analytics.

---

# Protocol Buffers for Analytics Engine
This is the main documentation about the Protocol Buffers for the Analytics Engine which includes server and client types.

## Server
The analytics engine will need a seperate gRPC instance running as the product grows. So, you will need to provide a `/grpc` endpoint, this is supported in **Arisu** and **charted-server**, so, you can just register it and it'll just work.

To provide a server for it (if you do wish to use the Analytics engine), we have server libraries for:

- [Kotlin](https://github.com/Noelware/analytics-server-kotlin)
- [Go](https://github.com/Noelware/analytics-server-go)

So you can just drop and place it and it'll run the service in the background with the rest of the HTTP endpoints you build.

To include the protobufs, you will need to create a submodule for it:

```ignore name=.gitmodules
[submodule "protos"]
    path = protos
    url = https://github.com/Noelware/analytics-protobufs.git
```

and then do `git submodule update` and you should have the protobufs available.

## Client
The **client** can request to the server if needed and retrieve information about the server itself. We have client libraries for:

- [C#](https://github.com/Noelware/analytics-client-csharp)
- [Go](https://github.com/Noelware/analytics-client-go)

## Services
### Analytics

> **Namespace**: Connection
>
> **Source**: https://github.com/Noelware/analytics-protobufs/blob/master/connection.proto

This is the main service that will be requested on the [server](#server). It exports the following data points:

### rpc ConnectionAck([ConnectionAckEvent](#connectionackevent)) -> [ConnectionAckResponse](#connectionackresponse)
Simple data point to check if the server is healthy or not.

### rpc RetrieveStats([RetrieveStatsEvent](#retrievestatsevent)) -> [RetrieveStatsResponse](#retrievestatsresponse)
This is the main data point that will be requested every interval you set. Since all instance UUIDs are unique and tied to a Noelware account, you don't have to worry about people looking at the statistics even if it doesn't show private data.

### rpc GetInstanceUUID([GetInstanceUUIDEvent](#getinstanceuuidevent)) -> [GetInstanceUUIDResponse](#getinstanceuuidresponse)
Returns the instance UUID that is stored in the Analytics database.

## Types
### ConnectionAckEvent
> **Namespace**: Connection
>
> **Source**: https://github.com/Noelware/analytics-protobufs/blob/master/connection.proto#L36

This is the main event message that is sent when requesting. This doesn't expose any properties.

### ConnectionAckResponse
> **Namespace**: Connection
>
> **Source**: https://github.com/Noelware/analytics-protobufs/blob/master/connection.proto#L38-L41

| Field       | Type  | Description                                                             |
| ----------- | ----- | ----------------------------------------------------------------------- |
| `connected` | bool  | If we are connected or not, if the request fails, this will be `false`. |
| `ping`      | float | Returns the ping from **server -> client** that was used.               |

### RetrieveStatsEvent
> **Namespace**: Connection
>
> **Source**: https://github.com/Noelware/analytics-protobufs/blob/master/connection.proto#L43

This is the main event message that is sent when requesting to `RetrieveStats`. This doesn't expose any properties.

### RetrieveStatsResponse
> **Namespace**: Connection
>
> **Source**: https://github.com/Noelware/analytics-protobufs/blob/master/connection.proto#L45-L51

This is the main response that is sent to the client when `RetrieveStats` was requested.

| Field          | Type   | Description                                                                           |
| -------------- | ------ | ------------------------------------------------------------------------------------- |
| `product`      | string | Returns the product that was requested. This can be `tsubaki` or `charted-server`.    |
| `version`      | string | Returns the version the product is running on.                                        |
| `commitSha`    | string | Returns the commit hash the product is using.                                         |
| `buildDate`    | string | Returns a **ISO8601**-compliant date string of the build date.                        |
| `buildFlavour` | string | Returns the build flavour the product is running on. Values can be `docker` or `git.` |

### GetInstanceUUIDEvent
> **Namespace**: Connection
>
> **Source**: https://github.com/Noelware/analytics-protobufs/blob/master/connection.proto#L53

This is the main event message that is sent when requesting to `GetInstanceUUID`. This doesn't expose any properties.

### GetInstanceUUIDResponse
> **Namespace**: Connection
>
> **Source**: https://github.com/Noelware/analytics-protobufs/blob/master/connection.proto#L55-57

This is the main response the client will send when `GetInstanceUUID` was requested.

| Field  | Type   | Description                        |
| ------ | ------ | ---------------------------------- |
| `uuid` | string | Returns the UUID of this instance. |
