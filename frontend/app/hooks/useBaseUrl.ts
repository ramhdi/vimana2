export function useBaseUrl(): string {
    const isProd = import.meta.env.PROD; // Vite's way of checking production mode

    return isProd ? "/vimana2" : "http://localhost:8081";
}
