# OpenApi Documentation
##Running Locally Backend
```bash
   cargo run
   ```
- http://127.0.0.1:3001/api-docs/openapi.json  
{
  "openapi": "3.0.3",
  "info": {
    "title": "backend",
    "description": "",
    "license": {
      "name": ""
    },
    "version": "0.1.0"
  },
  "paths": {
    "/login": {
      "post": {
        "tags": [
          "handlers"
        ],
        "summary": "Login a user and get authentication token",
        "operationId": "login_handler",
        "requestBody": {
          "content": {
            "application/json": {
              "schema": {
                "$ref": "#/components/schemas/LoginPayload"
              }
            }
          },
          "required": true
        },
        "responses": {
          "200": {
            "description": "Login successful",
            "content": {
              "application/json": {
                "schema": {
                  "$ref": "#/components/schemas/TokenResponse"
                }
              }
            }
          },
          "401": {
            "description": "Invalid credentials"
          },
          "500": {
            "description": "Internal server error"
          }
        }
      }
    },
    "/register": {
      "post": {
        "tags": [
          "handlers"
        ],
        "summary": "Register a new user",
        "operationId": "register_handler",
        "requestBody": {
          "content": {
            "application/json": {
              "schema": {
                "$ref": "#/components/schemas/RegisterPayload"
              }
            }
          },
          "required": true
        },
        "responses": {
          "200": {
            "description": "User registered successfully",
            "content": {
              "application/json": {
                "schema": {
                  "$ref": "#/components/schemas/User"
                }
              }
            }
          },
          "500": {
            "description": "Internal server error"
          }
        }
      }
    },
    "/todos": {
      "get": {
        "tags": [
          "handlers"
        ],
        "summary": "Get all todos for the authenticated user",
        "description": "Filter todos by completed status and search by title",
        "operationId": "get_all_todos_handler",
        "parameters": [
          {
            "name": "completed",
            "in": "path",
            "required": true,
            "schema": {
              "type": "boolean",
              "nullable": true
            }
          },
          {
            "name": "search",
            "in": "path",
            "required": true,
            "schema": {
              "type": "string",
              "nullable": true
            }
          }
        ],
        "responses": {
          "200": {
            "description": "List of todos",
            "content": {
              "application/json": {
                "schema": {
                  "type": "array",
                  "items": {
                    "$ref": "#/components/schemas/Todo"
                  }
                }
              }
            }
          },
          "401": {
            "description": "Unauthorized"
          },
          "500": {
            "description": "Internal server error"
          }
        },
        "security": [
          {
            "bearerAuth": []
          }
        ]
      },
      "post": {
        "tags": [
          "handlers"
        ],
        "summary": "Create a new todo",
        "operationId": "create_todo_handler",
        "requestBody": {
          "content": {
            "application/json": {
              "schema": {
                "$ref": "#/components/schemas/NewTodo"
              }
            }
          },
          "required": true
        },
        "responses": {
          "200": {
            "description": "Todo created successfully",
            "content": {
              "application/json": {
                "schema": {
                  "$ref": "#/components/schemas/Todo"
                }
              }
            }
          },
          "401": {
            "description": "Unauthorized"
          },
          "500": {
            "description": "Internal server error"
          }
        },
        "security": [
          {
            "bearerAuth": []
          }
        ]
      }
    },
    "/todos/{id}": {
      "get": {
        "tags": [
          "handlers"
        ],
        "summary": "Get a specific todo by ID",
        "operationId": "get_todo_handler",
        "parameters": [
          {
            "name": "id",
            "in": "path",
            "description": "Todo ID",
            "required": true,
            "schema": {
              "type": "integer",
              "format": "int32"
            }
          }
        ],
        "responses": {
          "200": {
            "description": "Todo found",
            "content": {
              "application/json": {
                "schema": {
                  "$ref": "#/components/schemas/Todo"
                }
              }
            }
          },
          "401": {
            "description": "Unauthorized"
          },
          "404": {
            "description": "Todo not found"
          },
          "500": {
            "description": "Internal server error"
          }
        },
        "security": [
          {
            "bearerAuth": []
          }
        ]
      },
      "put": {
        "tags": [
          "handlers"
        ],
        "summary": "Update a todo",
        "operationId": "update_todo_handler",
        "parameters": [
          {
            "name": "id",
            "in": "path",
            "description": "Todo ID",
            "required": true,
            "schema": {
              "type": "integer",
              "format": "int32"
            }
          }
        ],
        "requestBody": {
          "content": {
            "application/json": {
              "schema": {
                "$ref": "#/components/schemas/UpdateTodo"
              }
            }
          },
          "required": true
        },
        "responses": {
          "200": {
            "description": "Todo updated successfully",
            "content": {
              "application/json": {
                "schema": {
                  "$ref": "#/components/schemas/Todo"
                }
              }
            }
          },
          "401": {
            "description": "Unauthorized"
          },
          "404": {
            "description": "Todo not found or not owned by you"
          },
          "500": {
            "description": "Internal server error"
          }
        },
        "security": [
          {
            "bearerAuth": []
          }
        ]
      },
      "delete": {
        "tags": [
          "handlers"
        ],
        "summary": "Delete a todo",
        "operationId": "delete_todo_handler",
        "parameters": [
          {
            "name": "id",
            "in": "path",
            "description": "Todo ID",
            "required": true,
            "schema": {
              "type": "integer",
              "format": "int32"
            }
          }
        ],
        "responses": {
          "204": {
            "description": "Todo deleted successfully"
          },
          "401": {
            "description": "Unauthorized"
          },
          "404": {
            "description": "Todo not found or not owned by you"
          },
          "500": {
            "description": "Internal server error"
          }
        },
        "security": [
          {
            "bearerAuth": []
          }
        ]
      }
    }
  },
  "components": {
    "schemas": {
      "LoginPayload": {
        "type": "object",
        "required": [
          "username",
          "password"
        ],
        "properties": {
          "password": {
            "type": "string",
            "example": "password123"
          },
          "username": {
            "type": "string",
            "example": "john_doe"
          }
        }
      },
      "NewTodo": {
        "type": "object",
        "required": [
          "title"
        ],
        "properties": {
          "completed": {
            "type": "boolean",
            "example": false,
            "nullable": true
          },
          "title": {
            "type": "string",
            "example": "Buy groceries"
          }
        }
      },
      "RegisterPayload": {
        "type": "object",
        "required": [
          "username",
          "password"
        ],
        "properties": {
          "password": {
            "type": "string",
            "example": "password123"
          },
          "username": {
            "type": "string",
            "example": "john_doe"
          }
        }
      },
      "Todo": {
        "type": "object",
        "required": [
          "id",
          "title",
          "completed",
          "user_id"
        ],
        "properties": {
          "completed": {
            "type": "boolean",
            "example": false
          },
          "id": {
            "type": "integer",
            "format": "int32",
            "example": 1
          },
          "title": {
            "type": "string",
            "example": "Buy groceries"
          },
          "user_id": {
            "type": "integer",
            "format": "int32",
            "example": 1
          }
        }
      },
      "TodoQueryParams": {
        "type": "object",
        "properties": {
          "completed": {
            "type": "boolean",
            "example": true,
            "nullable": true
          },
          "search": {
            "type": "string",
            "example": "grocery",
            "nullable": true
          }
        }
      },
      "TokenResponse": {
        "type": "object",
        "required": [
          "token"
        ],
        "properties": {
          "token": {
            "type": "string",
            "example": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
          }
        }
      },
      "UpdateTodo": {
        "type": "object",
        "properties": {
          "completed": {
            "type": "boolean",
            "example": true,
            "nullable": true
          },
          "title": {
            "type": "string",
            "example": "Buy more groceries",
            "nullable": true
          }
        }
      },
      "User": {
        "type": "object",
        "required": [
          "id",
          "username",
          "password"
        ],
        "properties": {
          "id": {
            "type": "integer",
            "format": "int32",
            "example": 1
          },
          "password": {
            "type": "string",
            "example": "password123",
            "writeOnly": true
          },
          "username": {
            "type": "string",
            "example": "john_doe"
          }
        }
      }
    }
  },
  "tags": [
    {
      "name": "todos",
      "description": "Todo management endpoints"
    },
    {
      "name": "auth",
      "description": "Authentication endpoints"
    }
  ]
}
