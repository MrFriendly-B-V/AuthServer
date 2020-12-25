package nl.thedutchmc.authserver.springcontrollers;

import org.json.JSONObject;
import org.springframework.http.MediaType;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RequestMethod;
import org.springframework.web.bind.annotation.RequestParam;
import org.springframework.web.bind.annotation.RestController;

import nl.thedutchmc.authserver.App;
import nl.thedutchmc.authserver.User;
import nl.thedutchmc.authserver.auth.ApiManager;
import nl.thedutchmc.authserver.auth.SessionManager;

@RestController
@RequestMapping("/oauth")
public class PostController {
	
	// /oauth/session
	@PostMapping("session")
	public String session(@RequestBody String body) {		
		JSONObject bodyJson = new JSONObject(body);
		String sessionId = bodyJson.getString("session_id");
		
		boolean isAuthenticated = new SessionManager().sessionValid(sessionId);
		
		if(isAuthenticated) {
			return "OK";
		}
		
		return "Invalid session";
	}
	
	@RequestMapping(value = "token", method = RequestMethod.POST, produces = MediaType.APPLICATION_JSON_VALUE)
	public String token(@RequestParam(required = false) String sessionId, @RequestParam(required = false) String userId, @RequestParam String apiToken) {
		if(!ApiManager.isApiTokenValid(apiToken)) {
			JSONObject returnJson = new JSONObject();
			returnJson.put("status", 403);
			returnJson.put("sessionId", sessionId);
			returnJson.put("message", "Supplied apiToken is not valid.");
			
			return returnJson.toString();
		}
		
		User user = null;
		if(sessionId != null) {
			user = new SessionManager().getUserForSessionId(sessionId);
		}
		
		if(userId != null) {
			user = App.userMap.get(userId);
		}
		
		if(user != null) {
			JSONObject responseJson = new JSONObject(); 
			
			responseJson.put("status", 200);
			responseJson.put("token", user.getToken());
			responseJson.put("id", user.getId());
			responseJson.put("email", user.getEmail());
			
			return responseJson.toString();
		}
		
		JSONObject responseJson = new JSONObject();
		responseJson.put("status", 404);
		responseJson.put("sessionId", sessionId);
		responseJson.put("message", "No user with that sessionId exists.");
		
		return responseJson.toString();
	}
}
