import axios from "axios";

class NetworkService {
    static async Get() {
        const response = await axios.get("/api/ping");
        return response.data;
    }
}

export default NetworkService;