VERSION = 1.13.2
OUTPUT_DIR = kubernetes-apis

.PHONY: gen-client
gen-client: gen fix

.PHONY: gen
gen:
	openapi-generator generate --skip-validate-spec \
		-i https://raw.githubusercontent.com/kubernetes/kubernetes/v${VERSION}/api/openapi-spec/swagger.json \
		-g rust --output ${OUTPUT_DIR} \
		-DpackageName=${OUTPUT_DIR} -DpackageVersion=${VERSION} --library=reqwest

.PHONY: fix
fix:
	sh fixcode.sh
