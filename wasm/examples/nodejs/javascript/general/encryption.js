const velkar = require('../../../../nodejs/velkar');

velkar.initConsolePanicHook();

(async () => {

    let encrypted = velkar.encryptXChaCha20Poly1305("my message", "my_password");
    console.log("encrypted:", encrypted);
    let decrypted = velkar.decryptXChaCha20Poly1305(encrypted, "my_password");
    console.log("decrypted:", decrypted);

})();
