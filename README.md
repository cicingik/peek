<h1 align="center">peek</h1>
<p align="center">
  <em> Peeking key, value from configmaps or secrets resource.</em>
</p>

## Usage

```shell
⇒  kubectl peek --help
Peeking your key or/and value from configmap or/and secret

Usage: kubectl-peek [OPTIONS] [RESOURCE] [NAME]

Arguments:
  [RESOURCE]  Kind of resource [possible values: configmaps, secrets]
  [NAME]      Resource name

Options:
  -c, --context <CONTEXT>      Name of the kubeconfig context target. For override current context
  -n, --namespace <NAMESPACE>  Name of namespace target. Ignored if flag -A is present
  -k, --key <KEY>              Key name data resources
  -v, --value <VALUE>          Value data resources
  -A, --all                    If present, list the requested resource(s) across all namespaces
  -o, --output <OUTPUT>        Format output [default: table] [possible values: table, yaml, json]
  -h, --help                   Print help
```
## Peeking
```shell
⇒  k peek -n default -k MONGO_HOST -c cluster-production
  +==============+==============+=====================================+======================+==============+
  |     Kind     |  Namespace   |                Name                 |         Key          |    Value     |
  +==============+==============+=====================================+======================+==============+
  |  Configmaps  |  default     |  image-restoration-rest-config      |  MONGO_HOST          |  10.184.0.6  |
  +--------------+--------------+-------------------------------------+----------------------+--------------+
  |  Configmaps  |  default     |  kai-rest-config                    |  KAI_MONGO_HOST      |  10.184.0.6  |
  +--------------+--------------+-------------------------------------+----------------------+--------------+
  |  Configmaps  |  default     |  karin-rest-config                  |  KARIN_MONGO_HOST    |  10.184.0.6  |
  +--------------+--------------+-------------------------------------+----------------------+--------------+
  |  Configmaps  |  default     |  image-service-rest-config          |  MRSATAN_MONGO_HOST  |  10.184.0.6  |
  +==============+==============+=====================================+======================+==============+
```

```shell
⇒  k peek -n default -k MONGO_HOST -c cluster-production -o yaml
- kind: Configmaps
  name: image-restoration-rest-config
  namespace: default
  key: MONGO_HOST
  value: 10.184.0.6
- kind: Configmaps
  name: kai-rest-config
  namespace: default
  key: KAI_MONGO_HOST
  value: 10.184.0.6
- kind: Configmaps
  name: karin-rest-config
  namespace: default
  key: KARIN_MONGO_HOST
  value: 10.184.0.6
- kind: Configmaps
  name: image-service-rest-config
  namespace: default
  key: MRSATAN_MONGO_HOST
  value: 10.184.0.6
```

```shell
⇒  k peek -n default -k MONGO_HOST -c cluster-production -o json | jq .
[
  {
    "kind": "Configmaps",
    "name": "image-restoration-rest-config",
    "namespace": "default",
    "key": "MONGO_HOST",
    "value": "10.184.0.6"
  },
  {
    "kind": "Configmaps",
    "name": "kai-rest-config",
    "namespace": "default",
    "key": "KAI_MONGO_HOST",
    "value": "10.184.0.6"
  },
  {
    "kind": "Configmaps",
    "name": "karin-rest-config",
    "namespace": "default",
    "key": "KARIN_MONGO_HOST",
    "value": "10.184.0.6"
  },
  {
    "kind": "Configmaps",
    "name": "image-service-rest-config",
    "namespace": "default",
    "key": "MRSATAN_MONGO_HOST",
    "value": "10.184.0.6"
  }
]

```

## License

MIT  [©cicingik](https://github.com/cicingik)
