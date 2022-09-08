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

## Retrieve Node
`GET /node/{node.id}`

Retrieves a node by its id.

## Create Node
`POST /node/new`

Creates a new node. Returns the new node.

### Body Parameters
| Name | Type | Description |
| ---- | ---- | ----------- |
| `title` | `string` | The title of the node. |
| `body` | `string` | The body of the node. |

## Update Node
`PATCH /node/{node.id}`

Updates a node by its id. Returns the newly updated node.

### Body Parameters
| Name | Type | Default | Description |
| ---- | ---- | ------- | ----------- |
| `title?` | `string` | absent | Updates the title of the node if present. |
| `body?` | `string` | absent | Updates the body of the node if present. |

## Delete Node
`DELETE /node/{node.id}`

Deletes a node by its id. Returns a `204` on success.

