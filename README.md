# 🛡️ Thangail
_Geofencing helper for Mikrotik_

## Why?
Because someone is **NOT** welcome here

## How?
https://thangail.link

### Self-hosting
Run image and expose service, say, at `http://thangail.lan:8080`
```sh
docker run --rm -p 8080:80 ghcr.io/orhideous/thangail:master
```
Then, on MikroTik:
```sh
# Add firewall rules
/ip firewall raw add chain=prerouting action=drop src-address-list=thangail
/ipv6 firewall raw add chain=prerouting action=drop src-address-list=thangail
# Download and import IPv4 rules 
/tool fetch url="http://thangail.lan:8080/api/v0/list?country=cn&name=thangail&timeout=60d&version=v4" dst-path=v4.list
/import v4.list
/file remove v4.list
# Download and import IPv6 rules 
/tool fetch url="http://thangail.lan:8080/api/v0/list?country=cn&name=thangail&timeout=60d&version=v6" dst-path=v6.list
/import v6.list
/file remove v6.list
```
Rinse and repeat every 60 days, and keep an eye on memory/CPU usage.

## Q&A

### Geofencing entire countries is unethical!
So what?

### I don't trust this.
Good, you shouldn't. Self-host it.

### I'd like to help.
PRs are welcome.

### What is the source of country IP blocks?

RIR, compiled by Marcel Bischoff here: [herrbischoff/country-ip-blocks](https://github.com/herrbischoff/country-ip-blocks).
