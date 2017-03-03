# hynl-indent sublime text plugin
import sublime, sublime_plugin  
#import subprocess
from subprocess import Popen, PIPE, STDOUT
class HtmlIndentCommand(sublime_plugin.TextCommand):  
    def run(self, edit):
        region = sublime.Region(0, self.view.size())
        if not region.empty():  
            # Get the selected text  
            selected = bytes(self.view.substr(region), 'UTF-8')
            p = Popen(['html-indent'], stdout=PIPE, stdin=PIPE, stderr=STDOUT)    
            grep_stdout = p.communicate(input=selected)[0]
            output = grep_stdout.decode()
            self.view.replace(edit, region, output)
