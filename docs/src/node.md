# Node Resource
The Node, the basic entity of Ruina, is something with a `title` and `body`.
That's about it for the essentials. While the `body` is typically formatted in
Markdown, the `body` can be any generic data.

## Node Object

### Node Structure
| Name | Type | Description |
| ---- | ---- | ----------- |
| `id` | `id` | The unique identifier of the node. |
| `title` | `string` | The title of the node. |
| `body` | `string` | The actual text content of the node. |
| `parents?` | `string` | The parents of the node. |
| `children?` | `string` | The children of the node. |

## List Nodes
`GET /nodes`

Retrieves a list of nodes that are part of a single space. Pagination is
enforced, since a space can have upwards of one-thousand nodes.

### Query Parameters
| Name | Type | Default | Description |
| ---- | ---- | ------- | ----------- |
| `page` | `int` | The page number to examine. |
| `limit` | `int` | `20` | How many nodes to return in a single request. This number can be `1-20`. |

## Get Node
`GET /node/{node.id}`

Retrieves a node by its id.

