import API_URL from "../settings";

export const newRegistration = async (data, errorDiv) => {
  const myHeaders = new Headers({
    "Content-Type": "application/json",
  });

  const fetchConfig = {
    method: "POST",
    headers: myHeaders,
    cache: "no-cache",
    body: JSON.stringify(data),
  };

  const res = await fetch(`${API_URL}/registration`, fetchConfig);

  const json = await res.json();
  let result = JSON.stringify(json);
  errorDiv.classList.remove("error");
  console.log(result);
};