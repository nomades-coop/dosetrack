import API_URL from "../settings";

/// Obtiene un usuario de dosetrack de la base de datos de dosetrack segun su email de auth0
export const getUser = async (email) => {

  let user

  const myHeaders = new Headers({
    "Content-Type": "application/json",
  });

  const fetchConfig = {
    method: "GET",
    headers: myHeaders,
    cache: "no-cache",
  };

  const res = await fetch(
    `${API_URL}/user/email/${email}/`
  );

  if (res.ok) {
    user = await res.json();
  } else {
    console.log(res)
  }

  return user
}

/// Crea un nuevo usuario en dosetrack segun los datos de auth0
export const newUser = async (user) => {

  const myHeaders = new Headers({
    "Content-Type": "application/json",
  });

  const fetchConfig = {
    method: "POST",
    headers: myHeaders,
    cache: "no-cache",
  };

  const res = await fetch(
    `${API_URL}/user/`
  );

  const new_user = await res.json();

  if (res.ok) {
    return new_user
  } else {
    return false;
  }
}

