import UIKit

class ViewController: UIViewController {
    
    @IBOutlet var actionLabel: UILabel?
    
    var session = session_new()

    override func viewDidLoad() {
        super.viewDidLoad()
//        let string = session_action(self.session!, "Klaus")?.takeRetainedValue()
//        let s = string as NSString
//        self.actionLabel?.text = s as String
        
        // This works but leaks. It would require an additional session_destroy_action(something)
        // call to work properly which would clean up in rust
        let cstring = session_action2(self.session!, "Klaus")!
        let str = String(cString: cstring, encoding: .utf8)
        self.actionLabel?.text = str
    }


}

