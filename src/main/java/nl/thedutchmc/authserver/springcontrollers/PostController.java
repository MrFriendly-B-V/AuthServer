package nl.thedutchmc.authserver.springcontrollers;

import org.json.JSONObject;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

import nl.thedutchmc.authserver.session.SessionManager;

@RestController
@RequestMapping("/oauth")
public class PostController {
	
	// /oauth/session
	@PostMapping("session")
	public String session(@RequestBody String body) {
		//App.logDebug(body);
		
		JSONObject bodyJson = new JSONObject(body);
		String sessionId = bodyJson.getString("session_id");
		
		boolean isAuthenticated = new SessionManager().sessionValid(sessionId);
		
		if(isAuthenticated) {
			return "OK";
		}
		
		return "Invalid session";
	}	
}
