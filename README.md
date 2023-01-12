| ENV VAR | DESCRIPTION | EXAMPLE | DEFAULT |
|---------|-------------|---------|---------|
|CONFIG_FILE|Url to configuration file (Optional)|my_config.yml|users.yml|
|BIND_ADDR|Address listening on (Optional)|0.0.0.0|0.0.0.0|
|BIND_PORT|Port listening on (Optional)|8989|80|
|SSL_CERTIFICATE|Server certificate (Optional)|cert.pem|None|
|SSL_CERTIFICATE_KEY|Server certificate private key(Optional)|private.key|None|

TODO: ENV VAR TO CLEAR DATABASE ON STARTUP

TODO: FIX SHARED CONFIG ID in users.yml

https://github.com/Eugeny/tabby/blob/master/tabby-settings/src/services/configSync.service.ts
https://github.com/Eugeny/tabby-web/blob/4dab0c1dbf489c1ec1757822ac25ebd699ccd171/backend/tabby/app/models.py#L7
