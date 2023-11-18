import axios from "axios";

class NetworkService {
    static async Get(url) {
        const response = await axios.get(url);
        return response.data;
    }
}

export default NetworkService;