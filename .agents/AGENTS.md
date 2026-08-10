# Secret Management Rules

When working on this repository, strictly adhere to the following rules regarding secrets and credentials:

## REAL SECRET
❌ Jangan pernah masuk source code
❌ Jangan masuk test fixture
❌ Jangan masuk commit
❌ Jangan masuk issue/PR
❌ Jangan masuk screenshot
❌ Jangan masuk log

## SYNTHETIC FIXTURE
✅ Boleh untuk testing
✅ Harus jelas non-functional
✅ Tidak menggunakan credential-looking value yang tidak diperlukan
✅ Jangan menyerupai credential production secara berlebihan

## LOCAL CONFIG
✅ .env / environment variable
✅ .gitignore
✅ contoh konfigurasi → .env.example
❌ jangan commit nilai asli
