use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use tokio::sync::oneshot::channel;

pub async fn hash_password(password: String) -> String {
    let (send, recv) = channel();

    rayon::spawn(move || {
        let salt = SaltString::generate(&mut OsRng);

        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(password.as_ref(), &salt)
            .unwrap()
            .to_string();

        let _ = send.send(password_hash);
    });

    recv.await.unwrap()
}

pub async fn verify_password(password: String, hash: String) -> bool {
    let (send, recv) = channel();

    rayon::spawn(move || {
        let parsed_hash = PasswordHash::new(&hash).unwrap();
        let result = Argon2::default().verify_password(password.as_ref(), &parsed_hash);

        let _ = send.send(result);
    });

    recv.await.unwrap().is_ok()
}
