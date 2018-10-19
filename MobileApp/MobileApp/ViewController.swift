import UIKit

class ViewController: UIViewController {
    
    @IBOutlet var actionLabel: UILabel?

    override func viewDidLoad() {
        super.viewDidLoad()

        // Create a session
        let session = session_new()

        // Call the action method
        let string = session_action(session!, "Hello World")!.takeRetainedValue()

        // Apply string to UI
        let s = string as NSString
        self.actionLabel?.text = s as String
    }

}

